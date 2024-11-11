# Lesezeichen Editor

I initially developed the application in Pascal with the Lazarus IDE back in 2017.
Since then, it has operated flawlessly as my daily tool, continuing to be the best solution for my needs.
Although it's straightforward, I realized I wasn't taking advantage of the subtle features, such as hidden menus, quality-of-life enhancements, and text editor integration.
Consequently, I chose to reimplement only the most frequently used features in this Rust version.

The only reason for the existence of this program is for me to be able to add new bookmarks faster (and for trying out gtk4 with rust...).
There is no reason for me to create a more complex editor because I already have one like that. It's called Notepad++.

- Do not overengineer just because you can.
- Speed up or otherwise automate processes that are going to be used often.
- Keep your sensitive data offline.
- Sometimes doing less can mean more.

I thought about using an SQL database and fancy dynamic features. Stupid. The tool is supposed to be for me alone.
It is my personal bookmark collection. I don't want to be held down by some framework or database for which I would need a program just to read it.
So I started deleting and began with a simple HTML file as a bookmark list. Markdown is also not a bad idea when working with obsidian, as many people do, but it has its disadvantages.
Pure HTML, on the other hand, can be displayed in any web browser directly. The webbrowser itself brings many useful features and search functions to the table.
That's already more than what I need. Only the browser bookmark functions don't convince me. They are slow and lose their performance after a mere 50 items.
Also, they store a lot of useless data that I did not even give my agreement to. It also slows down the start duration of the browser. I hate for things to be slow or unresponsive, but that's what browser bookmarks tend to be like.
On the other hand, today's browsers have perfected handling big websites. So let's change the bookmarks to a website.
Now I can have tens of thousands of bookmarks without any noticeable performance hit. At the same time, I delete any addons or bookmarks in my browser that might slow down the user experience.
If possible, I will recompile my browser with the least possible number of features. Finally I will have just one bookmark in my browser's toolbar, and that's the html-bookmark-list.  
It's convenient to copy the file to a USB drive, allowing me to carry it with me and access my bookmarks on any PC with a USB port (e.g., university computer lab, a friend's computer, etc.) even without an internet connection.
