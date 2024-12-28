/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { basicSetup } from 'codemirror';
import { sql } from '@codemirror/lang-sql';
import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags } from '@lezer/highlight';
import { autocompletion } from '@codemirror/autocomplete';
import { Diagnostic, lintGutter, linter } from '@codemirror/lint';

// Define your custom theme
const myCustomTheme = EditorView.theme({
  '&': {
    backgroundColor: '#252525',
    color: '#E5E5E5',
    fontSize: '16px',
    fontFamily: 'Fira Code, monospace',
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
    backgroundColor: '#373737',
  },
  '.cm-line': {
    paddingLeft: '1px', // Add padding to text lines
  },
  '.cm-activeLine': {
    backgroundColor: '#252525', // No highlight for the active line
  },
  '.cm-completionHint': {
    backgroundColor: '#333333',
    color: '#E5E5E5',
    borderRadius: '5px',
    boxShadow: '0px 4px 8px rgba(0, 0, 0, 0.5)',
    fontSize: '14px',
  },
  '.cm-completionItem': {
    padding: '5px 10px',
    cursor: 'pointer',
  },
  '.cm-completionItem.cm-highlight': {
    backgroundColor: '#FF79C6',
    color: '#252525',
  },
});

const sqlHighlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: '#FF79C6' }, // Soft pink for keywords (e.g., SELECT, FROM)
  { tag: tags.literal, color: '#50FA7B' }, // Subtle green for string literals
  { tag: tags.number, color: '#F1FA8C' }, // Muted yellow for numbers
  { tag: tags.variableName, color: '#8BE9FD' }, // Calm light blue for variable names
  { tag: tags.operator, color: '#878787FF' }, // Soft red for operators
  { tag: tags.comment, color: '#6272A4', fontStyle: 'italic' }, // Soft blue for comments
  { tag: tags.tagName, color: '#BD93F9' }, // Muted purple for tag names (e.g., table names)
  { tag: tags.className, color: '#F8F8F2' }, // Soft white for class names
]);

interface SQLEditortProps {
  datasetsList: any[];
  dataframeMetadataList: any[];
  setQuery: (val: any) => void;
}

const SQLEditor: React.FC<SQLEditortProps> = ({
  datasetsList,
  dataframeMetadataList,
  setQuery,
}) => {
  // Function to get completions (including variables)
  const customCompletionSource = (context: any) => {
    const word = context.matchBefore(/\w*/); // Match a word (e.g., variable)
    if (!word) return null;

    const datasetVariables = datasetsList
      .filter((dataset) => dataset.file_name.startsWith(word.text)) // Filter based on typed text
      .map((dataset) => ({
        label: dataset.file_name,
        type: 'variable',
      }));

    const dataframeVariables = dataframeMetadataList
      .filter((dataframe) => dataframe.name.startsWith(word.text)) // Filter based on typed text
      .map((dataframe) => ({
        label: dataframe.name,
        type: 'variable',
      }));

    const allVariables = [...datasetVariables, ...dataframeVariables];

    return {
      from: word.from,
      options: allVariables,
    };
  };

  const customLinter = (view: EditorView): Diagnostic[] => {
    const diagnostics: Diagnostic[] = [];

    const doc = view.state.doc;
    const text = doc.toString();

    // Check for undeclared variables (e.g., dataset names or dataframe names)
    const variablePattern = /\b(?:data\w+)\b/g; // Match variable patterns (e.g., data1, data2)
    let match;
    while ((match = variablePattern.exec(text)) !== null) {
      const variable = match[0];
      // Check if the variable is in the list of defined variables
      const isValidVariable = datasetsList.some(
        (dataset) => dataset.file_name === variable,
      );

      if (!isValidVariable) {
        diagnostics.push({
          from: doc.lineAt(match.index).from, // Line and column of the error
          to: doc.lineAt(match.index + variable.length).to,
          severity: 'error', // Error severity level
          message: `Unknown variable: ${variable}`, // Error message
        });
      }
    }

    // You can add more checks here for invalid SQL syntax

    return diagnostics;
  };

  const [value, setValue] = useState<string>('SELECT * FROM table;');
  const [capturedData, setCapturedData] = useState<any>(null);

  const handleChange = (val: string, viewUpdate: any) => {
    setValue(val);

    let updatedQuery = val;

    // Match all dataset/file names inside the SQL query
    const datasetsInQuery: any[] = [];
    datasetsList.forEach((dataset) => {
      const regex = new RegExp(`\\b${dataset.file_name}\\b`, 'g');
      const matches = [...val.matchAll(regex)];

      // If the dataset is used in the query, add it to the datasetsInQuery array
      matches.forEach(() => {
        if (!datasetsInQuery.some((d) => d.id === dataset.id)) {
          datasetsInQuery.push({
            id: dataset.id,
            data_type: 'dataset',
          });
        }

        // Update the query to include double quotes around the dataset name
        updatedQuery = updatedQuery.replace(regex, `"${dataset.file_name}"`);
      });
    });

    // Remove newlines and extra whitespace from the query
    updatedQuery = updatedQuery.replace(/\s+/g, ' ').trim();

    // Store the captured data with multiple datasets or dataframes
    if (datasetsInQuery.length > 0) {
      const captured = {
        inputs: datasetsInQuery,
        sql_query: updatedQuery,
      };

      // Store the captured data
      setCapturedData(captured);
      setQuery(captured);
    }
  };

  return (
    <CodeMirror
      className='w-full h-full p-4'
      value={value}
      extensions={[
        basicSetup,
        sql(),
        myCustomTheme,
        syntaxHighlighting(sqlHighlightStyle),
        autocompletion({
          override: [customCompletionSource],
        }),
        linter(customLinter), // Attach the custom linter
        lintGutter(), // Enable gutter for displaying linting errors
      ]}
      onChange={handleChange}
    />
  );
};

export default SQLEditor;
