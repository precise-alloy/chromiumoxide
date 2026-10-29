use chromiumoxide_cdp::cdp::browser_protocol::emulation::{
    ScreenOrientation, ScreenOrientationType, SetDeviceMetricsOverrideParams,
    SetTouchEmulationEnabledParams,
};
use chromiumoxide_types::Method;

use crate::cmd::CommandChain;
use crate::handler::viewport::Viewport;
use std::time::Duration;

#[derive(Debug)]
pub struct EmulationManager {
    pub emulating_mobile: bool,
    pub has_touch: bool,
    pub needs_reload: bool,
    pub request_timeout: Duration,
}

impl EmulationManager {
    pub fn new(request_timeout: Duration) -> Self {
        Self {
            emulating_mobile: false,
            has_touch: false,
            needs_reload: false,
            request_timeout,
        }
    }

    pub fn init_commands(&mut self, viewport: &Viewport) -> CommandChain {
        let orientation = if viewport.is_landscape {
            ScreenOrientation::new(ScreenOrientationType::LandscapePrimary, 90)
        } else {
            ScreenOrientation::new(ScreenOrientationType::PortraitPrimary, 0)
        };

        let set_device = SetDeviceMetricsOverrideParams::builder()
            .mobile(viewport.emulating_mobile)
            .width(viewport.width)
            .height(viewport.height)
            .device_scale_factor(viewport.device_scale_factor.unwrap_or(1.))
            .screen_orientation(orientation)
            .build()
            .unwrap();

        let set_touch = SetTouchEmulationEnabledParams::new(viewport.has_touch);

        let chain = CommandChain::new(
            vec![
                (
                    set_device.identifier(),
                    serde_json::to_value(set_device).unwrap(),
                ),
                (
                    set_touch.identifier(),
                    serde_json::to_value(set_touch).unwrap(),
                ),
            ],
            self.request_timeout,
        );

        self.needs_reload = self.emulating_mobile != viewport.emulating_mobile
            || self.has_touch != viewport.has_touch;
        self.emulating_mobile = viewport.emulating_mobile;
        self.has_touch = viewport.has_touch;
        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_commands_respect_viewport_touch_setting() {
        let mut manager = EmulationManager::new(Duration::from_secs(5));
        let mut chain = manager.init_commands(&Viewport {
            has_touch: false,
            ..Default::default()
        });
        let mut commands = Vec::new();
        while let std::task::Poll::Ready(Some(Ok((method, params)))) =
            chain.poll(std::time::Instant::now())
        {
            commands.push((method.to_string(), params));
            chain.received_response(&method);
        }
        let touch = commands
            .iter()
            .find(|(m, _)| m == "Emulation.setTouchEmulationEnabled")
            .expect("touch command");
        assert_eq!(touch.1["enabled"], false);
    }
}
