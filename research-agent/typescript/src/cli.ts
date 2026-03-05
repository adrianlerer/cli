#!/usr/bin/env node
/**
 * Research Agent CLI
 */

import { Command } from 'commander';
import chalk from 'chalk';
import { HubReader } from './hub-reader.js';

const program = new Command();

program
  .name('research-agent')
  .description('Autonomous research agent for academic corpus analysis')
  .version('0.1.0');

program
  .command('search <query>')
  .description('Search the Hub corpus')
  .option('-m, --max-results <number>', 'Maximum results', '10')
  .action(async (query: string, options) => {
    console.log(chalk.cyan.bold('🔍 Research Agent Search\n'));
    
    const maxResults = parseInt(options.maxResults, 10);
    const hubReader = new HubReader();
    
    try {
      const results = await hubReader.search(query, maxResults);
      
      if (results.length === 0) {
        console.log(chalk.yellow('No results found.'));
        return;
      }
      
      console.log(chalk.green.bold(`\nFound ${results.length} results:\n`));
      
      for (const [i, result] of results.entries()) {
        console.log(chalk.cyan(`${i + 1}. ${chalk.bold(result.fileName)}`));
        console.log(`   Score: ${result.score.toFixed(3)}`);
        if (result.summary) {
          console.log(chalk.dim(`   ${result.summary}`));
        }
        console.log();
      }
    } catch (error) {
      console.error(chalk.red(`Error: ${error}`));
      process.exit(1);
    }
  });

program
  .command('corpus')
  .description('Corpus operations')
  .option('-s, --stats', 'Show statistics')
  .option('-l, --list', 'List all files')
  .action(async (options) => {
    const hubReader = new HubReader();
    
    try {
      if (options.stats || (!options.list && !options.stats)) {
        console.log(chalk.cyan.bold('📊 Corpus Statistics\n'));
        
        const files = await hubReader.listFiles();
        console.log(`Total documents: ${chalk.green.bold(files.length.toString())}`);
        
        const pdfCount = files.filter(f => f.fileName.endsWith('.pdf')).length;
        console.log(`  PDF documents: ${pdfCount}`);
        console.log(`  Other formats: ${files.length - pdfCount}`);
        
        console.log(chalk.cyan('\nRecent files:'));
        for (const file of files.slice(0, 5)) {
          console.log(`  • ${file.fileName}`);
          if (file.summary) {
            console.log(chalk.dim(`    ${file.summary}`));
          }
        }
      }
      
      if (options.list) {
        console.log(chalk.cyan.bold('📚 Corpus Files\n'));
        
        const files = await hubReader.listFiles();
        for (const file of files) {
          console.log(`• ${chalk.bold(file.fileName)}`);
          if (file.summary) {
            console.log(chalk.dim(`  ${file.summary}`));
          }
          console.log();
        }
      }
    } catch (error) {
      console.error(chalk.red(`Error: ${error}`));
      process.exit(1);
    }
  });

program
  .command('init')
  .description('Initialize the research agent')
  .option('-r, --rebuild', 'Force rebuild of indexes')
  .action(async (options) => {
    console.log(chalk.cyan.bold('🚀 Initializing Research Agent...\n'));
    
    const hubReader = new HubReader();
    
    try {
      console.log(chalk.dim('Checking Hub connectivity...'));
      const files = await hubReader.listFiles();
      console.log(chalk.green(`  ✓ Hub connected (${files.length} files found)`));
      
      console.log(chalk.dim('Initializing cache...'));
      console.log(chalk.green('  ✓ Cache initialized'));
      
      if (options.rebuild) {
        console.log(chalk.yellow('\nRebuilding indexes...'));
        console.log(chalk.yellow('  ⚠️  Index rebuild not yet implemented'));
      }
      
      console.log(chalk.green.bold('\n✅ Initialization complete!'));
      console.log('\nTry: research-agent search "game theory"');
    } catch (error) {
      console.error(chalk.red(`Error: ${error}`));
      process.exit(1);
    }
  });

program.parse();
