<script lang="ts">
	// Component Data
	export let datetimeUtc;

	// Renders the UTC ISO 8601 datetime as a locale-specific local time
	const renderDatetime = function (datetimeUtcStr) {
		const parsedDate = new Date(datetimeUtcStr); // Technically not a good idea, but it should be fine
		const currentDate = new Date();

		// Display the time if it's currently the same day, otherwise display the date
		if (
			parsedDate.getFullYear() == currentDate.getFullYear() &&
			parsedDate.getMonth() == currentDate.getMonth() &&
			parsedDate.getDate() == currentDate.getDate()
		)
			return parsedDate.toLocaleTimeString();
		else return parsedDate.toLocaleDateString();
	};
</script>

<time datetime={datetimeUtc} title={datetimeUtc}>{renderDatetime(datetimeUtc)}</time>
