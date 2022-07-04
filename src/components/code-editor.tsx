import React from 'react';
import { CodeEditor, Language } from '@patternfly/react-code-editor';

export const CodeEditorBasic: React.FunctionComponent = () => {
    const onEditorDidMount = (editor: { getValue: () => any; layout: () => void; focus: () => void; }, monaco: { editor: { getModels: () => { updateOptions: (arg0: { tabSize: number; }) => void; }[]; }; }) => {
        // eslint-disable-next-line no-console
        console.log(editor.getValue());
        editor.layout();
        editor.focus();
        monaco.editor.getModels()[0].updateOptions({ tabSize: 5 });
    };

    const onChange = (value: any) => {
        // eslint-disable-next-line no-console
        console.log(value);
    };

    return (
        <>
            <CodeEditor
                isDarkTheme={true}
                isLineNumbersVisible={true}
                isReadOnly={false}
                isMinimapVisible={true}
                isLanguageLabelVisible
                code="Some example content"
                onChange={onChange}
                language={Language.javascript}
                onEditorDidMount={onEditorDidMount}
                height="400px"
            />
        </>
    );
};
