from pwn import *
objects = ["ABC"]
objects_num = objects.__len__()
objects_string = reduce(lambda x, y: x + p32(y.__len__()) + y, objects, "")
objects_total_length = objects_string.__len__()
animation = p32(0)
binary = p32(objects_num)
binary += p32(objects_total_length)
binary += animation
binary += objects_string
open("test_files/object_exists_and_no_animation0", "w").write(binary)
