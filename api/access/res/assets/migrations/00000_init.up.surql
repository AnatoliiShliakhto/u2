BEGIN TRANSACTION;

LET $sa_login = 'root';
LET $sa_password = 'root';

DEFINE FUNCTION OVERWRITE fn::metadata::new($by: option<record>) {
    LET $time_now = time::unix();
    RETURN {
        created_by: $by,
        created_at: $time_now,
        updated_by: $by,
        updated_at: $time_now
    };
};

DEFINE TABLE users SCHEMAFULL TYPE NORMAL;
DEFINE FIELD login ON TABLE users TYPE string;
DEFINE FIELD password ON TABLE users TYPE option<string>;
DEFINE FIELD blocked ON TABLE users TYPE bool DEFAULT false;
DEFINE FIELD metadata ON TABLE users TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::new();
DEFINE FIELD metadata.updated_at ON TABLE users TYPE int VALUE time::unix();
DEFINE INDEX idx_users_login ON TABLE users COLUMNS login UNIQUE;

DEFINE TABLE refresh_tokens SCHEMAFULL TYPE NORMAL;
DEFINE FIELD issued_at ON TABLE refresh_tokens TYPE int VALUE time::unix();
DEFINE FIELD expiration_at ON TABLE refresh_tokens TYPE int DEFAULT time::unix() + 1296000;
DEFINE FIELD device ON TABLE refresh_tokens TYPE string DEFAULT 'web';

DEFINE TABLE rel_user_refresh_tokens SCHEMAFULL TYPE RELATION IN users OUT refresh_tokens;
DEFINE INDEX idx_rel_user_refresh_tokens ON TABLE rel_user_refresh_tokens COLUMNS in, out UNIQUE;

DEFINE TABLE permissions SCHEMAFULL;
DEFINE FIELD name ON TABLE permissions TYPE string;
DEFINE FIELD metadata ON TABLE permissions TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::new();
DEFINE FIELD metadata.updated_at ON TABLE permissions TYPE int VALUE time::unix();

DEFINE TABLE groups SCHEMAFULL TYPE NORMAL;
DEFINE FIELD name ON TABLE groups TYPE string;
DEFINE FIELD metadata ON TABLE groups TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::new();
DEFINE FIELD metadata.updated_at ON TABLE groups TYPE int VALUE time::unix();

DEFINE TABLE rel_group_permissions SCHEMAFULL TYPE RELATION IN groups OUT permissions;
DEFINE FIELD capabilities ON TABLE rel_group_permissions TYPE int DEFAULT 0;
DEFINE FIELD metadata ON TABLE rel_group_permissions TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::new();
DEFINE FIELD metadata.updated_at ON TABLE rel_group_permissions TYPE int VALUE time::unix();
DEFINE INDEX idx_rel_group_permissions ON TABLE rel_group_permissions COLUMNS in, out UNIQUE;

DEFINE TABLE rel_user_groups SCHEMAFULL TYPE RELATION IN users OUT groups;
DEFINE FIELD metadata ON TABLE rel_user_groups TYPE {
    created_at: int,
    updated_at: int,
    created_by: option<record>,
    updated_by: option<record>,
} DEFAULT fn::metadata::new();
DEFINE FIELD metadata.updated_at ON TABLE rel_user_groups TYPE int VALUE time::unix();
DEFINE INDEX idx_rel_user_groups ON TABLE rel_user_groups COLUMNS in, out UNIQUE;

LET $sa_rec = CREATE ONLY users CONTENT {
    login: $sa_login,
    password: crypto::argon2::generate($sa_password)
} RETURN VALUE id;

CREATE ONLY permissions:0 CONTENT {
    name: 'system',
    metadata: fn::metadata::new($sa_rec)
};
CREATE ONLY permissions:1 CONTENT {
    name: 'access',
    metadata: fn::metadata::new($sa_rec)
};
CREATE ONLY permissions:2 CONTENT {
    name: 'audit',
    metadata: fn::metadata::new($sa_rec)
};

CREATE ONLY groups:0 CONTENT {
    name: 'Administrators',
    metadata: fn::metadata::new($sa_rec)
};

RELATE groups:0->rel_group_permissions->permissions:0 CONTENT {
    capabilities: 255,
    metadata: fn::metadata::new($sa_rec)
};
RELATE groups:0->rel_group_permissions->permissions:1 CONTENT {
    capabilities: 255,
    metadata: fn::metadata::new($sa_rec)
};
RELATE groups:0->rel_group_permissions->permissions:2 CONTENT {
    capabilities: 255,
    metadata: fn::metadata::new($sa_rec)
};

RELATE $sa_rec->rel_user_groups->groups:0 CONTENT {
    metadata: fn::metadata::new($sa_rec)
};

RETURN true;

COMMIT TRANSACTION;