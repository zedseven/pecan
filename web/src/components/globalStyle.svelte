<style global lang="scss">
	body {
		margin: 0;
		padding: 0;
		font-family: sans-serif;
		/* font-size: 1.1em; */
	}

	input,
	button {
		box-sizing: border-box;
	}
	button,
	input[type='submit'] {
		cursor: pointer;
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
	.italicised {
		font-style: italic;
	}
	.monospace {
		font-family: monospace, monospace;
	}
	.strikethrough {
		text-decoration: line-through;
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
	.largerFont {
		font-size: 1.25em;
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

	tr:not(.noHoverDarken):hover {
		background-color: #dddddd;
	}

	.redBorder {
		border: 2px solid red;
		border-radius: 3px;
	}
	.noRedBorder {
		border: 2px solid transparent;
		border-radius: 3px;
	}

	@media print {
		body,
		#content {
			margin: 0;
			padding: 0;
		}
		:global(.unprintable) {
			display: none;
		}
	}
</style>
