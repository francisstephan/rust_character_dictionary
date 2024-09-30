pub fn ziform() -> String {
    let form = r##"
	  <form hx-post="/zilist" hx-target="#content" hx-swap="innerHTML" >
		    <label for="carac">Character:</label>
			<enctype="application/x-www-form-urlencoded">
		    <input id="carac" name="carac" type="text" autofocus required minlength="1" maxlength="1">
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

pub fn zistringform() -> String {
    let form = r##"
	    <p id="formhead">Enter zi string to parse :</p>
        <form hx-post="/stringparse" hx-target="#content" hx-swap="innerHTML" >
			<enctype="application/x-www-form-urlencoded">
		    <input id="zistr" name="zistr" type="text" autofocus required size="50" minlength="1" maxlength="300">
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	    </form>
	"##;
    String::from(form)
}

pub fn pyform() -> String {
    let form = r##"
        <form hx-post="/pylist" hx-target="#content" hx-swap="innerHTML" >
		    <label for="pinyin">Pinyin+tone (using pattern ^[a-z,ü]+[0-4]?) :</label>
		    <input id="pinyin_ton" name="pinyin_ton" type="text" pattern="^[a-z,ü]+[0-4]?" autofocus>
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

pub fn addziform() -> String {
    // Cancel button : cf. https://alchemists.io/projects/htmx-remove
    let form = r##"
	  <form id="addziform" class="multi" hx-post="/addzi" hx-target="#content" hx-swap="innerHTML">
	      <p id="formhead">Add character to dictionary :</p>
		  <input type="hidden" id="id" name="id" value="0">
		  <label for="pinyin_ton">Pinyin+tone (using pattern ^[a-z,ü]+[0-4]) :</label>
		  <input id="pinyin_ton" name="pinyin_ton" type="text" required pattern="^[a-z,ü]+[0-4]" autofocus><br />
		  <label for="unicode">Unicode (hexa, using pattern ^[0-9,A-F]{4}):</label>
		  <input id="unicode" name="unicode" type="text" required pattern="^[0-9,A-F]{4}">
				<span id="viewcar"> </span><br />
		  <label for="sens">Meaning:</label>
		  <input id="sens" name="sens" type="text" required minlength="2" maxlength="60"><br />

		<button class="formbut" type="submit">Submit</button>
		<button class="formbut" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	  <script>
	    function displayChar(){
			var s= document.getElementById("unicode");
			document.getElementById("viewcar").innerHTML = String.fromCharCode(parseInt(s.value,16))	;
		}
	    document.getElementById("unicode").addEventListener("keyup", displayChar);
		document.getElementById("unicode").addEventListener("change", displayChar);
	  </script>
	"##;
    String::from(form)
}

pub fn selupdate() -> String {
    let form = r##"
	  <form hx-post="/updatezi" hx-target="#content" hx-swap="innerHTML" >
		    <label for="id">Id:</label>
		    <input id="id" name="id" required type="number" autofocus>
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

pub fn seldelete() -> String {
    let form = r##"
	  <form hx-post="/deletezi" hx-target="#content" hx-swap="innerHTML" >
		    <p>Enter Id of entry to delete :</p>
			<label for="id">Id:</label>
		    <input id="id" name="id" required type="number" autofocus>
		    <button class="menubouton" type="submit">Submit</button>
			<button class="menubouton" hx-get="/cancel" hx-target="#content" hx-swap="innerHTML">Cancel</button>
	  </form>
	  "##;
    String::from(form)
}
