trunk build --release --public-url wordle-rs/
git add dist && git commit -m "Redeploying to gh pages"
git subtree push --prefix dist origin gh-pages