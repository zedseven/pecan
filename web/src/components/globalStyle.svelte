<style global lang="scss">
	body {
		margin: 0;
		padding: 0;
		font-family: sans-serif;
		font-size: 1.1em;
	}

	a,
	a:hover,
	a:active,
	a:visited {
		color: #288bdd;
	}
	a {
		text-decoration: none;
	}
	a:hover,
	a:hover > * {
		text-decoration: underline;
	}
	a:active {
		font-weight: bold;
	}
	.altLink,
	.altLink:hover,
	.altLink:active,
	.altLink:visited {
		color: teal;
	}

	/* Adds slightly more space between neighbouring columns' text */
	td {
		padding: 1px 4px;
	}

	.block {
		display: block;
	}
	.clear {
		clear: both;
	}
	.centerContents {
		text-align: center;
	}
	.maxWidth {
		box-sizing: border-box;
		width: 100%;
	}
	.monospace {
		font-family: monospace, monospace;
	}
	.noSelect,
	th,
	label,
	h1,
	h2,
	h3,
	h4,
	h5,
	h6 {
		-webkit-user-select: none;
		-ms-user-select: none;
		user-select: none;
	}
	.slightlyLargerFont {
		font-size: 1.05em;
	}
	.slightlySmallerFont {
		font-size: 0.95em;
	}
	.smallerFont {
		font-size: 0.75em;
	}

	#content {
		padding: 0.6em;
	}

	@media print {
		:global(.unprintable) {
			display: none;
		}
	}
</style>
