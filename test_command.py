#!/usr/bin/python

import time

class TestCommand:
    def run(self):
        full_line_txt = "Hello this is just a full line text to test how it looks on full line cmd prmt."
        for i in xrange(10):
            time.sleep(0.5)
            print("{} {}".format(i, full_line_txt))
TestCommand().run()
