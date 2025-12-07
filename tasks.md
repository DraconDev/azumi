

- using it made me realize a flaw it may have that is such an odd one, that we put the script tag first and arguably this is a great system, if we used outer css then we can't validate that afaik, but on bigger files like homepage we have over hundred lines of styles, that can't even be collapsed, you just have to scroll past it
  - putting it at the bottom is not great either i imagine, 
  - we in the past tried the external css solution and it did work, albeit performance was more of a concern, but the problem with it was that the rust analyzer doesn't care about css, 
  - the current system works best if the file is not that long, and argubaly better to have them together instead of opening two different files, 
  - this is a non issue for shoter components, but for longer files it can be a bit of a hassle

# Done

-   action testing

-   make the lessons
