use crate::drivers::platform::Platform;

pub struct MqttClient {
    pub connected: bool,
}

impl MqttClient {
    pub fn new() -> Self {
        Self { connected: false }
    }

    pub async fn connect<P: Platform>(&mut self, platform: &mut P, broker: &str, port: u16) {
        platform.esp8266_mqtt_connect(broker, port, None, None).await;
        self.connected = true;
    }

    pub async fn publish<P: Platform>(&self, platform: &mut P, topic: &str, payload: &str) {
        if self.connected {
            platform.esp8266_mqtt_publish(topic, payload, false).await;
        }
    }

    pub async fn subscribe<P: Platform>(&mut self, platform: &mut P, topic: &str) {
        platform.esp8266_mqtt_subscribe(topic).await;
    }
}