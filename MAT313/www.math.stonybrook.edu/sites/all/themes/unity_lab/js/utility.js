jQuery(document).ready(function() {
	jQuery('.announcement-slides').cycle({
		fx: 		'fade',
   		speed:       500, 
    	        timeout:     6000, 
		pause:		true,
		slideResize: false,
		containerResize: true
	})
});
