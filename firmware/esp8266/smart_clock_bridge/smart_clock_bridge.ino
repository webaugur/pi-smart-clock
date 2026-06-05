/*
 * Smart Clock ESP8266 serial bridge
 *
 * Wire to Linux host UART (3.3V logic):
 *   ESP TX  -> host RX
 *   ESP RX  -> host TX
 *   GND     -> GND
 *
 * Line protocol (115200 8N1, newline terminated):
 *   PING -> PONG
 *   WIFI <ssid><TAB><password> -> WIFI OK <ip>
 *   MQTT_CONN <broker> <port> [<user><TAB><pass>] -> OK
 *   MQTT_PUB <topic> <retain:0|1> <len>\n<bytes> -> OK
 *   MQTT_SUB <topic> -> OK
 *   NTP <server> -> NTP OK <iso8601>
 *   HTTP_GET <url> -> HTTP OK <status> <len>\n<raw bytes>
 */

#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <PubSubClient.h>
#include <ESP8266HTTPClient.h>
#include <WiFiUdp.h>
#include <NTPClient.h>

const char *WIFI_SSID = "";
const char *WIFI_PASS = "";

WiFiClient wifiClient;
PubSubClient mqtt(wifiClient);
WiFiUDP ntpUdp;
NTPClient ntp(ntpUdp, "pool.ntp.org", 0, 60000);

String line;

void replyOK() { Serial.println("OK"); }
void replyERR(const String &msg) { Serial.print("ERR "); Serial.println(msg); }
void logMsg(const String &msg) { Serial.print("LOG "); Serial.println(msg); }

bool ensureWifi() {
  if (WiFi.status() == WL_CONNECTED) return true;
  if (strlen(WIFI_SSID) > 0) {
    WiFi.begin(WIFI_SSID, WIFI_PASS);
    for (int i = 0; i < 40 && WiFi.status() != WL_CONNECTED; i++) delay(250);
  }
  return WiFi.status() == WL_CONNECTED;
}

void connectWifiFromArgs(const String &args) {
  int tab = args.indexOf('\t');
  if (tab < 0) { replyERR("wifi args"); return; }
  String ssid = args.substring(0, tab);
  String pass = args.substring(tab + 1);
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid.c_str(), pass.c_str());
  for (int i = 0; i < 60 && WiFi.status() != WL_CONNECTED; i++) delay(250);
  if (WiFi.status() != WL_CONNECTED) { replyERR("wifi failed"); return; }
  Serial.print("WIFI OK ");
  Serial.println(WiFi.localIP());
}

void mqttCallback(char *topic, byte *payload, unsigned int length) {
  Serial.print("MQTT_MSG ");
  Serial.print(topic);
  Serial.print(' ');
  Serial.write(payload, length);
  Serial.println();
}

void connectMqtt(const String &args) {
  if (!ensureWifi()) { replyERR("no wifi"); return; }
  int sp1 = args.indexOf(' ');
  if (sp1 < 0) { replyERR("mqtt args"); return; }
  String broker = args.substring(0, sp1);
  String rest = args.substring(sp1 + 1);
  int sp2 = rest.indexOf(' ');
  int port = 1883;
  String creds = "";
  if (sp2 >= 0) {
    port = rest.substring(0, sp2).toInt();
    creds = rest.substring(sp2 + 1);
  } else {
    port = rest.toInt();
  }
  mqtt.setServer(broker.c_str(), port);
  mqtt.setCallback(mqttCallback);
  mqtt.setBufferSize(1024);
  const char *user = nullptr;
  const char *pass = nullptr;
  String userS, passS;
  int tab = creds.indexOf('\t');
  if (tab >= 0) {
    userS = creds.substring(0, tab);
    passS = creds.substring(tab + 1);
    user = userS.c_str();
    pass = passS.c_str();
  }
  if (!mqtt.connect("smart-clock-bridge", user, pass)) { replyERR("mqtt connect"); return; }
  replyOK();
}

void publishMqtt(const String &topic, int retain, int len) {
  if (!mqtt.connected()) { replyERR("mqtt down"); return; }
  uint8_t buf[1024];
  if (len > (int)sizeof(buf)) { replyERR("payload too large"); return; }
  int read = 0;
  unsigned long deadline = millis() + 5000;
  while (read < len && (long)(deadline - millis()) > 0) {
    while (Serial.available() && read < len) buf[read++] = Serial.read();
    if (read < len) delay(1);
  }
  if (read < len) { replyERR("mqtt payload timeout"); return; }
  mqtt.publish(topic.c_str(), buf, len, retain != 0);
  replyOK();
}

void subscribeMqtt(const String &topic) {
  if (!mqtt.connected()) { replyERR("mqtt down"); return; }
  mqtt.subscribe(topic.c_str());
  replyOK();
}

void runNtp(const String &server) {
  if (!ensureWifi()) { replyERR("no wifi"); return; }
  NTPClient client(ntpUdp, server.c_str(), 0, 60000);
  client.begin();
  client.forceUpdate();
  if (client.getEpochTime() == 0) { replyERR("ntp failed"); return; }
  Serial.print("NTP OK ");
  Serial.println(client.getEpochTime());
}

void runHttp(const String &url) {
  if (!ensureWifi()) { replyERR("no wifi"); return; }
  HTTPClient http;
  WiFiClient client;
  if (!http.begin(client, url)) { replyERR("http begin"); return; }
  http.setTimeout(20000);
  int code = http.GET();
  if (code <= 0) { replyERR("http get"); http.end(); return; }
  String body = http.getString();
  http.end();
  if (body.length() > 8192) { replyERR("http body too large"); return; }
  Serial.print("HTTP OK ");
  Serial.print(code);
  Serial.print(' ');
  Serial.println(body.length());
  Serial.write((const uint8_t *)body.c_str(), body.length());
}

void handleLine(const String &cmd) {
  if (cmd == "PING") { Serial.println("PONG"); return; }
  if (cmd.startsWith("WIFI ")) { connectWifiFromArgs(cmd.substring(5)); return; }
  if (cmd.startsWith("MQTT_CONN ")) { connectMqtt(cmd.substring(10)); return; }
  if (cmd.startsWith("MQTT_SUB ")) { subscribeMqtt(cmd.substring(9)); return; }
  if (cmd.startsWith("NTP ")) { runNtp(cmd.substring(4)); return; }
  if (cmd.startsWith("HTTP_GET ")) { runHttp(cmd.substring(9)); return; }
  if (cmd.startsWith("MQTT_PUB ")) {
    String rest = cmd.substring(9);
    int sp1 = rest.indexOf(' ');
    int sp2 = rest.indexOf(' ', sp1 + 1);
    if (sp1 < 0 || sp2 < 0) { replyERR("mqtt pub args"); return; }
    String topic = rest.substring(0, sp1);
    int retain = rest.substring(sp1 + 1, sp2).toInt();
    int len = rest.substring(sp2 + 1).toInt();
    publishMqtt(topic, retain, len);
    return;
  }
  replyERR("unknown cmd");
}

void setup() {
  Serial.begin(115200);
  Serial.setTimeout(50);
  line.reserve(256);
  if (strlen(WIFI_SSID) > 0) {
    WiFi.mode(WIFI_STA);
    WiFi.begin(WIFI_SSID, WIFI_PASS);
  }
  logMsg("smart-clock bridge ready");
}

void loop() {
  mqtt.loop();
  while (Serial.available()) {
    char c = Serial.read();
    if (c == '\n') {
      line.trim();
      if (line.length() > 0) handleLine(line);
      line = "";
    } else if (c != '\r') {
      line += c;
    }
  }
}