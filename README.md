# web-labs

To run web application locally on port 8080:
```sh
$ cargo run --bin web-labs
```

To create a new post:
```sh
$ cargo run --bin write_post
```

To publish the post:
```sh
$ cargo run --bin publish_post
```

To show all published posts:
```sh
$ cargo run --bin show_posts
```

To get title of a post:
```sh
$ cargo run --bin get_post num
```
where num is a post number, i.e. 1

To delete post:
```sh
$ cargo run --bin delete_post
```

