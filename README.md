# Letterboxed Solver

Just a quick letterboxed solver I wrote. Isn't guaranteed to find optimal solutions, but in my experience it mostly does. It is however severely limited by the quality of dictionary used. Included in this repo
is [this](https://github.com/outparse/english-dictionary-dataset) dictionary, because I found that it has enough words to solve it in 2-3 guesses almost all the time without producing words not accepted by letterboxed too often. However,
larger dictionaries might produce better results at the cost of needing to trim them manually to get rid of illegal words.

## Usage
execute ``cargo run letterboxed {Side1} {Side2} {Side3} {Side4}``
where {Side1}-{Side4} get replaced with the letters of the puzzle (order doesn't matter)

Example: ``cargo run letterboxed qts aoi ngu hpv``

## Dictionary

It is also possible to extract the legal words for the Letterboxed game of the current day by pasting the following code snippet into your console when opening letterboxed

```js
(function () {
	const origHas = Set.prototype.has;
	const origGet = Map.prototype.get;
	const origIncludes = Array.prototype.includes;

	window.__LB_CANDIDATES__ = new Set();

	Set.prototype.has = function (v) {
		if (this.size > 1000) {
			window.__LB_DICT__ = this;
		}
		return origHas.call(this, v);
	};

	Map.prototype.get = function (v) {
		if (this.size > 1000) {
			window.__LB_DICT__ = this;
		}
		return origGet.call(this, v);
	};

	Array.prototype.includes = function (v) {
		if (this.length > 1000) {
			window.__LB_DICT__ = this;
		}
		return origIncludes.call(this, v);
	};

	console.log("Hooks installed. Now enter any word.");
})();
```

then entering any word. The word list can then be extracted using this code

```js
copy(window.__LB_DICT__.join("\n"));
```

then simply paste it into the words.txt file replacing the original contents.
This code might stop working at any point as nytimes might update their website.