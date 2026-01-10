class Solution:
     def minBitFlips(self,start:int,goal:int)->int:
          totalAns=start ^ goal
          count=0
          while totalAns:
               count+=1
               totalAns >>=1
          return count
        