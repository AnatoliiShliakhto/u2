#![allow(dead_code)]
use ::bitflags::bitflags;

bitflags! {
    #[derive(Clone, Eq, PartialEq, Hash)]
    pub struct Capabilities: u8 {
        const NONE          = 0b0000_0000;

        // Basic permissions
        const CREATE        = 0b0000_0001;
        const VIEW          = 0b0000_0010;
        const UPDATE        = 0b0000_0100;
        const DELETE        = 0b0000_1000;

        const FULL          = 0b0000_1111;

        // Role-based permissions
        const USER          = 0b0001_0000;
        const EDITOR        = 0b0010_0000;
        const MANAGER       = 0b0100_0000;
        const ADMINISTRATOR = 0b1000_0000;

        const ALL           = 0b1111_0000;

        // Special roles
        const SA = Self::ALL.bits() | Self::FULL.bits();
    }
}

impl Capabilities {
    pub fn is_admin(&self) -> bool {
        self.contains(Self::ADMINISTRATOR)
    }

    pub fn is_manager(&self) -> bool {
        self.contains(Self::MANAGER)
    }

    pub fn is_editor(&self) -> bool {
        self.contains(Self::EDITOR)
    }

    pub fn is_user(&self) -> bool {
        self.contains(Self::USER)
    }

    pub fn can_create(&self) -> bool {
        self.contains(Self::CREATE)
    }

    pub fn can_read(&self) -> bool {
        self.contains(Self::VIEW)
    }

    pub fn can_write(&self) -> bool {
        self.contains(Self::UPDATE)
    }

    pub fn can_delete(&self) -> bool {
        self.contains(Self::DELETE)
    }
}
