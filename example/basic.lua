-- Filter example
-- GMX account 
account "gmx"
{   
   login { user = env 'GMX_EMAIL', pass = env 'GMX_PASS' }
   serv "imap.gmx.net"
   ssl
   auth "login"
   port "993"
}

-- Yahoo account
account "yahoo"
{
   login { user = env "YAHOO_EMAIL", pass = env 'YAHOO_PASS' }
   serv 'imap.mail.yahoo.com'
   auth "login"
   tls
}

-- Google account
account "google"
{
   login { user = env 'GOOGLE_EMAIL', pass = env 'GOOGLE_PASS' }
   serv "imap.gmail.com"
   auth "plain"
}

filter "slashdot"
{
   search 'gmx:INBOX'
   {
      from = 'slashdot',
   }
   mark "seen"
   copy 'google:INBOX'
   move 'gmx:Slashdot'
}

filter "github"
{
   search 'gmx:INBOX'
   {
      from 'github.com'
      unseen
   }
   
   copy 'google:news/Github'
   move 'gmx:Github'
}

filter "spam"
{
   search 'gmx:INBOX'
   {
      from = {'news.brgmedia.com', 'travel.hoteltravel-email.com'}
   }
   delete
}
