-- Simple Lua example
-- GMX account 
account "gmx"
{
   login { user = env 'GMX_EMAIL', pass = env 'GMX_PASS' },
   serv "imap.gmx.net",
   ssl,
   auth "login",
   port "993"
}

filter "github"
{
   search 'gmx:INBOX'
   {
      from 'github.com',
      unseen,
   },
   move 'gmx:Github'
}
