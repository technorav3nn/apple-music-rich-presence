function run(input) {
	const Music = Application("Music");
	const props = {
		state: "stopped",
	};
	if (!Music.running) {
		return JSON.stringify(props);
	}
	switch (input[0]) {
		case "stop":
			Music.stop();
			break;
		case "play":
			Music.play();
			break;
		case "pause":
			Music.pause();
			break;
		case "next":
			Music.nextTrack();
			break;
		case "previous":
			Music.previousTrack();
			break;
		default:
			break;
	}
	props.state = Music.playerState();
	if (["playing", "paused"].includes(props.state)) {
		props.id = Music.currentTrack().id();
		props.artist = Music.currentTrack().artist();
		props.album = Music.currentTrack().album();
		props.song = Music.currentTrack().name();
		props.duration = Music.currentTrack().duration();
		props.position = Music.playerPosition();
		props.rawArtwork = Music.currentTrack().artworks[0].rawData();
	}
	return JSON.stringify(props);
}
