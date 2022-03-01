function MenuBar() {
  return (
    <div className="App">
      <nav
        className="navbar navbar-expand-lg navbar-light"
        style={{ backgroundColor: "#e3f2fd" }}
      >
        <div className="container-fluid">
          <a className="navbar-brand" href="/">
            Online Judge
          </a>
          <ul className="navbar-nav me-auto mb-2 mb-lg-0">
            <li className="nav-item">
              <a
                className="nav-link active"
                aria-current="page"
                href="/problems"
              >
                problems
              </a>
            </li>
            <li className="nav-item">
              <a className="nav-link" href="/status">
                status
              </a>
            </li>
            <li className="nav-item">
              <a className="nav-link" href="/discuss">
                discuss
              </a>
            </li>
          </ul>
          <form className="d-flex">
            <input
              className="form-control me-2"
              type="search"
              placeholder="Search"
              aria-label="Search"
            />
            <button className="btn btn-outline-success" type="submit">
              Search
            </button>
          </form>
        </div>
      </nav>
    </div>
  );
}

export default MenuBar;
