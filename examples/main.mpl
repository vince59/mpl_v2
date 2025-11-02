123+30
zzzz ++++++ a                             bxxxxxxx     c

import /* // import
import // fkjglkjggfjhlk
import import
import /* dflkgldfkjgkl */
import

.1 9
 import
 fn
 main
 print
 println
 call
 to_str aa
 nl
 local
 true
 false "toto va à la plage"
 int
 float //test
 let
 for+
 to
 step
 next
 break (+-/)
 {
 }
 ,
 +
 -
 *
 / 0.2 07 .0 2.  bb aa_bb_cc123 1

import "lib\utils.mpl"
import "lib\unit.mpl"

/*
todo :
boucle for

for i = 1 to 10 step 1
  println(i)
  break
next


gestion des booleen
gestion des chaines
*/

main() {
  local int i
  local float f
  /*local string s = "toto"
  local bool b = true */

  let i = -5
  let f = 12.3 + -i
  print("Data types :",nl, "i = ",to_str(i),nl,"f = ",to_str(f),nl)

  print("Hello from mpl !",nl) // single line comment
  call hello_from_unit()

  /*
  multiple lines comment
  */
  print("x=", to_str(3.5),"y=",to_str(125.458),nl)
  call hello_from_utils()
  print("x=", to_str((40+4)/(2*2.3)-5.5),nl)
  print("L1", nl, "L2",nl)
  print("[","","]",nl)
  println("a")
  println("b")

  for i = 1 to 10 step 1
    println(to_str(i))
  next

}

