import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    // Get configuration
    const config = vscode.workspace.getConfiguration('rue');
    const cargoPath = config.get<string>('languageServer.path', 'cargo');
    const cargoArgs = config.get<string[]>('languageServer.args', [
        'run', '-p', 'rue-lsp', '--bin', 'rue-lsp'
    ]);

    // Server options
    const serverOptions: ServerOptions = {
        command: cargoPath,
        args: cargoArgs,
        options: {
            // Run from the rue project root directory
            cwd: findRueProjectRoot()
        }
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rue' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.rue')
        }
    };

    // Create and start the language client
    client = new LanguageClient(
        'rueLanguageServer',
        'Rue Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (this will also launch the server)
    client.start();

    // Register commands
    const disposable = vscode.commands.registerCommand('rue.restartLanguageServer', async () => {
        await client.stop();
        client.start();
        vscode.window.showInformationMessage('Rue Language Server restarted');
    });

    context.subscriptions.push(disposable);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function findRueProjectRoot(): string {
    // Try to find the rue project root by looking for Cargo.toml with rue workspace
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders) {
        for (const folder of workspaceFolders) {
            // Check if this looks like the rue project
            const cargoTomlPath = vscode.Uri.joinPath(folder.uri, 'Cargo.toml');
            try {
                const fs = require('fs');
                if (fs.existsSync(cargoTomlPath.fsPath)) {
                    const content = fs.readFileSync(cargoTomlPath.fsPath, 'utf8');
                    if (content.includes('rue-lsp')) {
                        return folder.uri.fsPath;
                    }
                }
            } catch (e) {
                // Continue searching
            }
        }
        // Fallback to first workspace folder
        return workspaceFolders[0].uri.fsPath;
    }
    
    // Fallback to current working directory
    return process.cwd();
}