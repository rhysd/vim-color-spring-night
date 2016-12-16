#!/usr/bin/env ruby

File.read(File.join(File.dirname(__dir__), 'colors', 'spring-night.vim'))
  .split("\n")
  .select{|s| s.start_with? "call s:hi('" }
  .map{|s| s =~ /^call s:hi\('([^']*)',/; $1}
  .group_by{|s| s}
  .each{|n, g| abort "'#{n}' duplicates!" if g.length > 1 }

puts "OK"
