/// A handle to local audio, present whether or not it is available.
#[derive(Default)]
pub struct LocalPlayback {
    #[cfg(feature = "local-playback")]
    device: Option<LocalDevice>,
    /// Kept so the device can be built again after a session drops.
    blueprint: Option<Blueprint>,
}

impl LocalPlayback {
    /// A handle that plays nothing, for remote-only operation.
    #[must_use]
    pub fn disabled() -> Self {
        Self::default()
    }

    /// Starts local audio, registering termusic as a Connect device.
    #[cfg_attr(not(feature = "local-playback"), expect(clippy::unused_async))]
    pub async fn start(blueprint: Blueprint) -> Result<Self, StartError> {
        #[cfg(feature = "local-playback")]
        {
            let device = LocalDevice::start(&blueprint).await?;
            Ok(Self {
                device: Some(device),
                blueprint: Some(blueprint),
            })
        }

        #[cfg(not(feature = "local-playback"))]
        {
            let _ = blueprint;
            Err(StartError::NotCompiledIn)
        }
    }

    /// Whether the audio session is still up.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        #[cfg(feature = "local-playback")]
        {
            self.device.as_ref().is_some_and(LocalDevice::is_alive)
        }

        #[cfg(not(feature = "local-playback"))]
        {
            false
        }
    }

    /// Whether a device was started and has since stopped on its own.
    #[must_use]
    pub fn has_died(&self) -> bool {
        self.is_running() && !self.is_alive()
    }

    /// The settings this device was built from, for building it again.
    #[must_use]
    pub fn blueprint(&self) -> Option<&Blueprint> {
        self.blueprint.as_ref()
    }

    /// The samples on their way to the speakers, if audio is local.
    #[must_use]
    pub fn tap(&self) -> SampleTap {
        #[cfg(feature = "local-playback")]
        {
            self.device
                .as_ref()
                .map_or_else(SampleTap::disconnected, LocalDevice::tap)
        }

        #[cfg(not(feature = "local-playback"))]
        {
            SampleTap::disconnected()
        }
    }

    /// Whether audio is coming out of this process.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.device_id().is_some()
    }

    /// The Spotify device id, as it appears in the device list.
    #[must_use]
    pub fn device_id(&self) -> Option<&str> {
        #[cfg(feature = "local-playback")]
        {
            self.device.as_ref().map(LocalDevice::device_id)
        }

        #[cfg(not(feature = "local-playback"))]
        {
            None
        }
    }

    /// The name this device announces itself under.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        #[cfg(feature = "local-playback")]
        {
            self.device.as_ref().map(LocalDevice::name)
        }

        #[cfg(not(feature = "local-playback"))]
        {
            None
        }
    }

    /// Starts the user's Liked Songs here.
    #[must_use]
    pub fn play_liked_songs(&self) -> bool {
        #[cfg(feature = "local-playback")]
        {
            match self.device.as_ref() {
                Some(device) => match device.play_liked_songs() {
                    Ok(()) => true,
                    Err(error) => {
                        tracing::warn!(%error, "could not start Liked Songs locally");
                        false
                    }
                },
                None => false,
            }
        }

        #[cfg(not(feature = "local-playback"))]
        {
            false
        }
    }

    /// Sets the volume on the local mixer, reporting whether it was taken.
    #[must_use]
    pub fn set_volume(&self, percent: u8) -> bool {
        #[cfg(feature = "local-playback")]
        {
            match self.device.as_ref() {
                Some(device) => match device.set_volume(percent) {
                    Ok(()) => true,
                    Err(error) => {
                        tracing::warn!(%error, "could not set the local volume");
                        false
                    }
                },
                None => false,
            }
        }

        #[cfg(not(feature = "local-playback"))]
        {
            let _ = percent;
            false
        }
    }

    /// Unregisters the device, if one is running.
    pub fn shutdown(&self) {
        #[cfg(feature = "local-playback")]
        if let Some(device) = self.device.as_ref() {
            device.shutdown();
        }
    }
}
