#![allow(dead_code)]
use ::bitflags::bitflags;

bitflags! {
    /// Represents user capabilities combining basic CRUD permissions with role-based access control.
    ///
    /// The lower 4 bits represent basic permissions (CRUD operations),
    /// while the upper 4 bits represent role-based permissions.
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Capabilities: u8 {
        /// No permissions
        const NONE = 0b0000_0000;

        // Basic CRUD permissions (bits 0-3)
        /// Permission to create resources
        const CREATE = 0b0000_0001;
        /// Permission to view/read resources
        const VIEW = 0b0000_0010;
        /// Permission to update/modify resources
        const UPDATE = 0b0000_0100;
        /// Permission to delete resources
        const DELETE = 0b0000_1000;

        /// All basic CRUD permissions combined
        const FULL = Self::CREATE.bits() | Self::VIEW.bits() | Self::UPDATE.bits() | Self::DELETE.bits();

        // Role-based permissions (bits 4-7)
        /// Standard user role
        const USER = 0b0001_0000;
        /// Editor role with content management capabilities
        const EDITOR = 0b0010_0000;
        /// Manager role with team management capabilities
        const MANAGER = 0b0100_0000;
        /// Administrator role with system-wide access
        const ADMINISTRATOR = 0b1000_0000;

        /// All role-based permissions combined
        const ALL = Self::USER.bits() | Self::EDITOR.bits() | Self::MANAGER.bits() | Self::ADMINISTRATOR.bits();

        /// Super Administrator with all permissions and roles
        const SA = Self::ALL.bits() | Self::FULL.bits();
    }
}

impl Capabilities {
    // Role checking methods

    /// Checks if the user has administrator privileges
    pub fn is_admin(&self) -> bool {
        self.has_role(Self::ADMINISTRATOR)
    }

    /// Checks if the user has manager privileges
    pub fn is_manager(&self) -> bool {
        self.has_role(Self::MANAGER)
    }

    /// Checks if the user has editor privileges
    pub fn is_editor(&self) -> bool {
        self.has_role(Self::EDITOR)
    }

    /// Checks if the user has basic user privileges
    pub fn is_user(&self) -> bool {
        self.has_role(Self::USER)
    }

    // Permission checking methods

    /// Checks if the user can create resources
    pub fn can_create(&self) -> bool {
        self.has_permission(Self::CREATE)
    }

    /// Checks if the user can view/read resources
    pub fn can_view(&self) -> bool {
        self.has_permission(Self::VIEW)
    }

    /// Checks if the user can update/modify resources
    pub fn can_write(&self) -> bool {
        self.has_permission(Self::UPDATE)
    }

    /// Checks if the user can delete resources
    pub fn can_delete(&self) -> bool {
        self.has_permission(Self::DELETE)
    }

    // Helper methods to reduce duplication

    /// Helper method to check if a specific role is present
    fn has_role(&self, role: Self) -> bool {
        self.contains(role)
    }

    /// Helper method to check if a specific permission is present
    fn has_permission(&self, permission: Self) -> bool {
        self.contains(permission)
    }
}
