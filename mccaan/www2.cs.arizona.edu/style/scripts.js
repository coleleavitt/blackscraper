<!--  

/* THIS SCRIPT EXTRACTS A NAME FROM THE URL, THEN LOOKS TO SEE IF THAT NAME 
MATCHES ANY ID IN SIDEBAR MENU. IF SO, IT SWAPS THE CLASS NAME FOR THE MATCHING 
MENU ITEM TO SET IT TO A HIGHLIGHTED STATE.*/


// object detection to set some browser vars cause we need to branch our code for IE5
isIE5 = ( document.all && !document.fireEvent && !window.opera )? true : false;
isIE = ( document.all )? true : false;
isDOM = ( !document.all && document.getElementById )? true : false;



window.onload = function()
{
    setClassOn("sidebarMenu", generateMenuID());
    
    // generates a menuID name from the page URL
    function generateMenuID()
    {
    
        var myPath = window.location.pathname;
        var pathArray = myPath.split("/");
        var numPathElements = pathArray.length;         
	var lastElement = pathArray[ numPathElements -1 ];
	if ( lastElement == "" || lastElement == "index.html")
	    lastElement = pathArray[ numPathElements -2 ];
        var menuID = parsePathElement( lastElement );

        
        // strip extension for a few common types of pages
        // and remove any underscores from the remainder
        function parsePathElement( pathElement )
        {            
            pathElement = pathElement.replace("~", "" );
            pathElement = pathElement.replace(".html", "" );
            pathElement = pathElement.replace(".shtml", "" );
            pathElement = pathElement.replace(".htm", "" );
            pathElement = pathElement.replace( /_/g , "" );    
            return pathElement;
        }
        
        return menuID;
    
    }        


    // picked up this script from sitepoint developer page
    // creates div on the fly to wrap our HR tag and then we style the divs 
    // with CSS to create pretty rules that work the same on all browsers
    function fixHr() 
    {   
        if (!document.getElementsByTagName) return; 
	    var hr = document.getElementsByTagName("hr");

        for (var i=0; i<hr.length; i++) 
        { 
            var newhr = hr[i]; 
            var wrapdiv = document.createElement('div');
            wrapdiv.className = 'line';  
            newhr.parentNode.replaceChild(wrapdiv, newhr);  
            wrapdiv.appendChild(newhr);  
        }  
    } 

    if ( !isIE5 ) fixHr();    
    
}



function printAddlYear()
{
    if ( !document.getElementById ) return;
    
    var myDateObj = new Date();
    var startYear = 2006;
    var currentYear = myDateObj.getFullYear();
    var yearString = ( currentYear > startYear )? " - " + currentYear: "";  
    
    document.write( yearString );
}



//  Highlight the global navigation entry for this page.

function setHighlight(name)
{
    setClassOn("globalNav", name);
    setClassOn("utilityNav", name);
}



//  Set class to "menuOn" for <li id=name>
//  if found by name within <ul id=lname>.

function setClassOn(lname, name)
{
    var elist = document.getElementById(lname);
    if ( elist )
    {    
	var listElements = elist.getElementsByTagName("li");
    }

    if ( listElements )
    {
	// loop thru. If there's a match,
	// set the class to our "on" menu state class
	for( var i = 0; i < listElements.length; i++ )
	{   
	    if( listElements[i].getAttribute("id") == name )
	    {
		listElements[i].setAttribute("class", "menuOn");	// isDOM
		listElements[i].setAttribute("className", "menuOn");	// isIE
	    }
	}    
    }
    
}



//  Generate HTML link for mailing to "uname@cs.arizona.edu".
//  Use the label if supplied, else label with the e-mail address.

function atcs(uname, label)
{
    var addr = uname + '@cs.arizona.edu';
    if ( !label ) label = addr;
    document.write('<a href="mailto:' + addr + '">' + label +'</a>' );
}



//  Generate HTML link for mailing to "uname@host.arizona.edu".
//  Use the label if supplied, else label with the e-mail address.

function atua(uname, host, label)
{
    var addr = uname + '@' + host + '.arizona.edu';
    if ( !label ) label = addr;
    document.write('<a href="mailto:' + addr + '">' + label +'</a>' );
}



//-->
