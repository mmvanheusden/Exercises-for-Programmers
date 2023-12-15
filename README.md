# Exercises-for-Programmers
My solutions to the 57 challenges in the book "Exercises for Programmers"

### **Objective**  
My objective isn't to just look up everything, but to really try to come up with solutions myself.  
I'll try to switch programming language from time-to-time, so I learn a bit from everything.

**Turning problems into Code**  
Another objective is to increase my problem-solving skills.  
The book has a nice chapter for that. It goes as follows:

* First, you gather information. You ask as many questions as you want. This is called "requirement gathering".  
  You figure out what features and bounds your program should have.
* Second, try writing out a [problem statement](https://en.wikipedia.org/wiki/Problem_statement).  
  For example:
  ```text
  Create a simple tip calculator.
  
  The program should ask how much % you want to tip (the tip rate), and the total amount of the bill.
  The program must then calculate the tip amount, and the sum of the tip and the bill.
  At the end it must display those values.
  ```
* Then, discover the inputs, processes, and outputs your program has.  
  This can be found out by analyzing the problem statement.  
  First, look at the nouns. In this example it goes as follows:  
  * tip
  * total amount
  * tip amount
  * bill sum
  * tip

  Then, take the verbs:
  * ask
  * calculate
  * display

  With this in mind, we can determine that out inputs, processes, and outputs look like this:
  * Inputs: bill amount, tip rate (% of total bill)
  * Processes: calculate the tip
  * Outputs: tip amount, bill sum  
  
* Now the cool part, writing the program in _[pseudocode](https://en.wikipedia.org/wiki/Pseudocode)_.  
  What is pseudocode? basically, you write the code out in English.
  It's like a "sketch" but for coding.
  Example:
  ```text
  TipCalculator:

  Init billAmount to 0
  Init tip to 0
  Init tipRate to 0
  Init total to 0
  
  Ask for billAmount with "What is the bill amount?"
  Ask for tipRate with "How much % of the bill do you want to tip?"
  
  convert billAmount to a number
  convert tipRate to a number
  
  tip = billAmount * (tipRate / 100)
  round bill up to nearest cent
  total = billAmount + tip
  
  display "Tip: " + tip + "€"
  display "Total: " + total + "€"
  ```
* Now, we can get to coding! remember, write everything down on paper!
