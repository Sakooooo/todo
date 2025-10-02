# todo - a poorly made todolist
todo is a poorly made todolist I made for myself.

todo is still a work in progress.

## Usage
### Creating folders
```
~> cd ~/todos
~/todos> todo init ./me
Name the task folder:
me
created task folder me at /home/myself/todos/me
~/todos> todo init ./work
Name the task folder:
work
created task folder me at /home/myself/todos/work
```

### Adding tasks
```
~/todos> todo add me -t "find the meaning of life"
Created category inbox in me at /home/myself/todos/me
Created task with ID 1 in inbox of me at /home/myself/todos/me
~/todos> todo add me -c goals -t "walk around"
Created category goals in me at /home/myself/todos/me
Created task with ID 1 in goals of me at /home/myself/todos/me
~/todos> todo add work -t "show up"
Created category inbox in me at /home/myself/todos/work
Created task with ID 1 in inbox of work at /home/myself/todos/work
~/todos> todo add work -t "check out"
Created task with ID 2 in inbox of work at /home/myself/todos/work
```

### Listing tasks
```
# you can also run todo ls!
~/todos> todo list 
Folders:
  me
      goals
          1 TODO walk around
      inbox
          1 TODO find the meaning of life
  work
      inbox
          1 TODO show up
          2 TODO check out
```

## Updating tasks
```
~/todos> todo set work inbox 2 -t "clock out"
Updated item ID 2 in category inbox of folder work.
~/todos> todo set me goals 1 -s done
Updated item ID 1 in category goals of folder me.
~/todos> todo set me inbox 1 -s in-progress
Updated item ID 1 in category goals of folder me.
~/todos> todo set work inbox 1 -s done
Updated item ID 1 in category inbox of folder work.
```

## Installation
<details>
<summary>Nix</summary>
TODO
</details>
