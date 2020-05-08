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
   print("LUA: login called:", l)
   return  function(t)
      print("LUA: ** login anonymous function called.")
   end
end

function serv(s)
   print("LUA: serv called:", s)
   return s
end

function ssl()
   print("LUA: ssl called.")
   return "ssl"
end

function auth(s)
   print("LUA: auth called:", s)
   return s
end

function port(p)
   print("LUA: port called:", p)
   return p
end

-- function env(f)
--    print("LUA: env called:", f)
--    return f
-- end

-- -- filter
-- function filter(f)
--    print("LUA: filter called: ", f)
--    return  function(t)
--       print("LUA: ** filter anonymous function called.")
--    end
-- end

function search(s)
   print("LUA: search called:", s)
   return  function(t)
      print("LUA: ** search anonymous function called for ", s)
      return t
   end
end

function from(f)
   print("LUA: from called:", f)
   return f
end

function move(m)
   print("LUA: move called:", m)
   return m
end

function copy(c)
   print("LUA: copy called:", c)
   return c
end
