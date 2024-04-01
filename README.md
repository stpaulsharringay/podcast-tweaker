This is a Vercel serverless function that takes the Wordpress output of our RSS feed (ie, <https://stpaulsharringay.com/wp-content/uploads/podcast.xml>), and transforms it slightly to:
1. Include the passage reference in the episode title
2. Put the speaker's name as the episode description

I've written this in Rust because I enjoy overcomplicating things. And the Rust RSS parser/generator seemed *way* better thought through than the JS one I found (although I'm sure there are better ones).

The Vercel function is aliased from <https://podcast.stpaulsharringay.com>.
