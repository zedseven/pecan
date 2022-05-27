<script lang="ts">
	// Constants
	const millisPerDay = 24 * 60 * 60 * 1000;

	// Component Data
	export let datetimeUtc;

	// Renders the UTC ISO 8601 datetime as a locale-specific local time
	const renderDatetime = function (datetimeUtcStr) {
		const parsedDate = new Date(datetimeUtcStr); // Technically not a good idea, but it should be fine
		const elapsedMillis = new Date() - parsedDate;

		// Display the time if it's been less than 24h, otherwise display the date
		if (elapsedMillis < millisPerDay) return parsedDate.toLocaleTimeString();
		else return parsedDate.toLocaleDateString();
	};
</script>

<time datetime={datetimeUtc} title={datetimeUtc}>{renderDatetime(datetimeUtc)}</time>
