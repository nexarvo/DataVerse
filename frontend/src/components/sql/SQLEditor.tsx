import React, { useState } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { basicSetup } from 'codemirror';
import { sql } from '@codemirror/lang-sql';
import { EditorView } from '@codemirror/view';
// import { tags } from '@lezer/highlight';
// import { HighlightStyle } from '@codemirror/language';

// Define your custom theme
const myCustomTheme = EditorView.theme({
  '&': {
    backgroundColor: '#252525',
    color: '#E5E5E5',
    fontSize: '16px',
    fontFamily: 'Arial, sans-serif',
  },
  '.cm-content': {
    backgroundColor: '#252525', // Editor background color
    color: '#8A8A8A', // Text color
    fontSize: '13px',
  },
  '.cm-gutters': {
    backgroundColor: '#252525', // Same background as editor
    color: '#E5E5E5', // Line number color
    border: 'none',
    fontSize: '12px',
  },
  '.cm-gutterElement': {
    backgroundColor: '#252525', // Ensure line numbers background is same as editor
  },
  '.cm-cursor': {
    borderLeftColor: '#fff', // Cursor color
  },
  '.cm-selectionBackground': {
    backgroundColor: '#252525', // No color change for selection
  },
  '.cm-line': {
    paddingLeft: '1px', // Add padding to text lines
  },
  '.cm-activeLine': {
    backgroundColor: '#252525', // No highlight for the active line
  },
});

// const myHighlightStyle = HighlightStyle.define([
//   { tag: tags.keyword, color: '#fc6' },
//   { tag: tags.comment, color: '#f5d', fontStyle: 'italic' },
// ]);

const SQLEditor: React.FC = () => {
  const [value, setValue] = useState<string>('SELECT * FROM table;');

  const handleChange = (value: string, viewUpdate: any) => {
    setValue(value);
  };

  return (
    <CodeMirror
      className='w-full h-full p-4'
      value={value}
      extensions={[basicSetup, sql(), myCustomTheme]}
      onChange={handleChange}
    />
  );
};

export default SQLEditor;
