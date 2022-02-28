function MenuBar() {
  return (
    <div className="App">
      <nav
        className="navbar navbar-expand-lg navbar-light"
        style={{ backgroundColor: "#e3f2fd" }}
      >
        <div class="container-fluid">
          <a class="navbar-brand" href="/">
            Online Judge
          </a>
          <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li class="nav-item">
              <a class="nav-link active" aria-current="page" href="/problems">
                problems
              </a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="/status">
                status
              </a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="/discuss">
                discuss
              </a>
            </li>
          </ul>
          <form class="d-flex">
            <input
              class="form-control me-2"
              type="search"
              placeholder="Search"
              aria-label="Search"
            />
            <button class="btn btn-outline-success" type="submit">
              Search
            </button>
          </form>
        </div>
      </nav>
    </div>
  );
}

export default MenuBar;