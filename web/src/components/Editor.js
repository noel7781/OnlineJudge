import React, { Fragment, Component } from "react";

import Editor from "react-simple-code-editor";
import Highlight, { defaultProps } from "prism-react-renderer";
import theme from "prism-react-renderer/themes/nightOwl";

const exampleCode = `
(function someDemo() {
  var test = "Hello World!";
  console.log(test);
})();

return () => <App />;
`;

const styles = {
  root: {
    boxSizing: "border-box",
    fontFamily: '"Dank Mono", "Fira Code", monospace',
    ...theme.plain,
  },
};

const EditorArea = ({ code, setCode, lang }) => {
  const highlight = (code) => (
    <Highlight {...defaultProps} theme={theme} code={code} language={lang}>
      {({ className, style, tokens, getLineProps, getTokenProps }) => (
        <Fragment>
          {tokens.map((line, i) => (
            <div {...getLineProps({ line, key: i })}>
              {line.map((token, key) => (
                <span {...getTokenProps({ token, key })} />
              ))}
            </div>
          ))}
        </Fragment>
      )}
    </Highlight>
  );

  return (
    <Editor
      value={code}
      onValueChange={setCode}
      highlight={highlight}
      padding={10}
      style={styles.root}
    />
  );
};
export default EditorArea;
