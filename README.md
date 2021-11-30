# nested_iter
Experiment with nested iterator


This is an exercise to teach myself how to use nested iterators 
(instead of a nested for loop), and do combined computation
with each of the loop indexes. The idea is to map the outer iterator
to the inner iterator. Since iterators are lazy, you must flatten
and for_each the result. You must also move the data since the
iterator can be executed after the parent is gone.

Why do this? Iterators handle borrows more cleanly than expanded
loops. 

Weither or not this is idiomatic is for others to decide.
