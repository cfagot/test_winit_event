This is aimple app to test some issue with winit and x11 on linux and differences with wayland.

There are 4 triangles. The left column is controlled by device events, the right column by window events. The top row are keyboard events, the bottom row are mouse events (left/right).

The demo has an intentional 10 ms sleep during render. If you set the key repeat to maximum in settings then it should repeat keys at a 1 ms clip, outpacing the frame rate.

On x11 this results in buffering of window key events (upper right triangle will keep rotation after you lift the key). On wayland it appears that key repeats do not happen so no buffering. Also note that wayland does not produce device key events so the upper-left triangle will not change.

Mouse events have no buffering issues in either x11 or wayland. This indicates the key buffering issue is not due to the app hogging the event loop since moouse events are produced at even a faster clip.

Another difference between x11 and wayland is that wayland does not produce device mouse events when the mouse is outside the window.

The key buffering issue can also be seen clearly in the console output. The app will print "***" in the new_events callback and "..." in the about_to_wait callback (these two calls bracket the callbacks from a single iteration of the event loop). Even with key repeats happening at 1 ms intervals there is never more than one window key event between the new_events and about_to_wait callbacks. And this isn't just for key repeats -- this also holds true if you mash the keyboard repeatedly. Furthermore, the device event key released events will happen on time, with streams of window event key events still streaming in one event per iteration through the event loop long after the device release event occurred. This feels very synchrounous, unlike the mouse events which are processed at the soonest possible time. Even if you bump the frame time up to a full second, the mouse events that occurred while you were sleeping will be processed before the next render.

Finally, note that this is rendering using the recommended pattern. Render in redraw_requested() and request a new redraw after the render. The 10 ms sleep is there to simulate a scene that renders around 100 fps. It is not uncommon for some games to render at 30 fps which results in buffering with more modest key repeat rates (the 1 ms/key rate is absurd, of course).