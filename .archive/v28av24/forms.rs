pub fn ziform() -> String {
    let form = r##"
	  <form hx-post="/zilist" hx-target="#content" hx-swap="innerHTML" >
		    <label for="carac">Character:</label>
			<enctype="application/x-www-form-urlencoded">
		    <input id="carac" name="carac" type="text" autofocus required minlength="1" maxlength="1">
		    <button class="menubouton" type="submit">Click to submit </button>
			<button class="menubouton" hx-get="/remove" hx-target="#content", hx-swap="innerHTML">Cancel</button>
	  </form>
	"##;
    String::from(form)
}
