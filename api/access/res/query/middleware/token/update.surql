UPDATE ONLY type::thing('tokens', <uuid> $token_id)
MERGE {
    expiration_at: $expiration_at
}
WHERE expiration_at > time::unix()
RETURN VALUE <string> id.id();