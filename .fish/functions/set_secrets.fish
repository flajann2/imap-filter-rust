function set_secrets -d "just for testing"
    set -x GMX_EMAIL "fred@notarealemailaddress.com"
    set -x GMX_PASS "123456_worst_password_ever"
    echo "export GMX_EMAIL=$GMX_EMAIL ; export GMX_PASS=$GMX_PASS ; "
end
