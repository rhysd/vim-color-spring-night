#!/usr/bin/env ruby

# XXX: Ruby on Travis CI is v1.9. __dir__ is unavailable.
File.foreach(File.join(File.dirname(File.dirname(__FILE__)), 'colors', 'spring-night.vim'))
  .select{|s| s.start_with?("hi ") || s.start_with?("exe 'hi' ") }
  .map{|s| s =~ (s.start_with?("hi ") ?  /^hi (\w+)/ : /^exe 'hi' '(\w+)'/); $1 }
  .group_by{|s| s}
  .each{|n, g| abort "'#{n}' duplicates!" if g.length > 1 }

puts "OK"
