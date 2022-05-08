<script lang="ts">
	// Component Data
	export let datetimeUtc;

	// Renders the UTC ISO 8601 datetime as a locale-specific local time
	const renderDatetime = function (datetimeUtcStr) {
		const parsedDate = new Date(datetimeUtcStr); // Technically not a good idea, but it should be fine
		return parsedDate.toLocaleDateString() + ' ' + parsedDate.toLocaleTimeString();
	};
</script>

<time datetime={datetimeUtc} title={datetimeUtc}>{renderDatetime(datetimeUtc)}</time>
