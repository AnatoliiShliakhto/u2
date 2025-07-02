BEGIN TRANSACTION;

LET $user_rec = type::thing('tokens', <uuid> $refresh_token)<-rel_user_tokens.in[0];

RETURN SELECT
    id.id() as id,
    (->rel_user_groups->groups->rel_group_permissions.{key: out.id(), val: capabilities})
        .map(|$item| [$item.key, $item.val]) as permissions
FROM ONLY $user_rec;

COMMIT TRANSACTION;