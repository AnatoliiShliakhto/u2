BEGIN TRANSACTION;

LET $refresh_token_rec = type::thing('tokens', rand::uuid::v4());
LET $user_rec = type::thing('users', $user_id);

CREATE ONLY $refresh_token_rec CONTENT {
    expiration_at: $expiration_at,
    device: $device
};

RELATE $user_rec->rel_user_tokens->$refresh_token_rec;

RETURN <string> $refresh_token_rec.id();

COMMIT TRANSACTION;