#import <UIKit/UIKit.h>

extern void mirui_start(void) __attribute__((noreturn));

int main(int argc, char *argv[]) {
    @autoreleasepool {
        mirui_start();
    }
}
