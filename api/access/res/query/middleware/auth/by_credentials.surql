SELECT
    id.id() as id,
    (->rel_user_groups->groups->rel_group_permissions.{key: out.id(), val: capabilities})
        .map(|$item| [$item.key, $item.val]) as permissions
FROM ONLY users
WHERE
    login = $login AND
    crypto::argon2::compare(password, $password)
LIMIT 1;