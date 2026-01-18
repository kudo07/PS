class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        sizeArr=len(nums)
        powerSubsets=1<<sizeArr
        ans=[[] for _ in range(powerSubsets)]
        for i in range(sizeArr):
            for j in range(powerSubsets):
                if((j>>i) & 1):
                    ans[j].append(nums[i])
        return ans