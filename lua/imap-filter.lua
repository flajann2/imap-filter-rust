-- helper lua DSL support for imap-filter.
-- Shall be automatically included for imap-filter
print("*** imap filter support script loaded ***")

-- function account(a)
--    print("LUA: account called.")
--    return  function(t)
--       print("LUA: ** account anonymous function called.")
--    end
-- end

function login(l)
   print("LUA: login called.")
   return  function(t)
      print("LUA: ** login anonymous function called.")
   end
end

function serv(s)
   print("LUA: serv called.")
end

function ssl()
   print("LUA: ssl called.")
end

function auth(s)
   print("LUA: auth called.")
end

function port(p)
   print("LUA: port called.")
end

function env(f)
   print("LUA: env called.")
end

-- filter
function filter(f)
   print("LUA: filter called.")
   return  function(t)
      print("LUA: ** filter anonymous function called.")
   end
end

function search(s)
   print("LUA: search called.")
   return  function(t)
      print("LUA: ** search anonymous function called.")
   end
end

function from(f)
   print("LUA: from called.")
end

function move(m)
   print("LUA: move called.")
end

function copy(c)
   print("LUA: copy called.")
end
