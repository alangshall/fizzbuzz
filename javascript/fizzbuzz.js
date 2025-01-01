var count = 0;
var fizz = 3;
var buzz = 5;

while (count <= 100)  {
  if (count % fizz == 0 && count % buzz == 0) console.log ("FizzBuzz");
  else if (count % fizz == 0) console.log("Fizz");
  else if (count % buzz == 0) console.log("Buzz");
  else console.log(count);
  count++;
}
