### Method
- The points are initially very far apart. They travel long distances in terms of pixels...about tens of thousands of pixels. This can't be reasonably captured in an image.
- Create a threshold ~100pixels. Whenever the width and height of the smallest square that contains all the points falls within this threshold we capture that instance and put it in an image.
- For `part1`, just run the simulation and captre all the appropriate instances and put the images in the `results` directory.
- For `part2`, manually go through the images and find the number of the image that contains the message. Use this id as a cli argument to find the time instant when the simulation reaches the state captured by the image.


### Tip
Run `cargo run --release -- --help` for tips

### Example
```shell
$ cargo run --release -- --part 1 --input input.txt

No. of images written: 69
Go through the images in 'result' directory and get the id of the correct one for part 2
```

Suppose the message appeared in image `23.gif`

```shell
$ cargo run --release -- --part 2 --input input.txt --id 23

Image of size 62x10 appeared @ time 10423s
```
