<script lang="ts">
  import { Bot, Send, Code, Bug, BarChart3, Lightbulb, Copy, ThumbsUp, ThumbsDown, Sparkles } from 'lucide-svelte'
  
  let messages = [
    {
      id: 1,
      type: 'assistant',
      content: 'Hello! I\'m your AI Copilot. I can help you with code generation, debugging, data analysis, and ML best practices. What would you like to work on today?',
      timestamp: new Date()
    }
  ]
  
  let currentMessage = ''
  let isTyping = false
  let selectedContext = 'general'
  
  const contextOptions = [
    { value: 'general', label: 'General Help', icon: Bot },
    { value: 'code', label: 'Code Generation', icon: Code },
    { value: 'debug', label: 'Debugging', icon: Bug },
    { value: 'analysis', label: 'Data Analysis', icon: BarChart3 },
    { value: 'optimization', label: 'Optimization', icon: Lightbulb }
  ]
  
  const quickPrompts = [
    'Generate a data preprocessing pipeline',
    'Debug this model training error',
    'Suggest feature engineering techniques',
    'Optimize hyperparameters for better performance',
    'Explain this machine learning concept',
    'Review my code for best practices'
  ]
  
  function sendMessage() {
    if (!currentMessage.trim()) return
    
    // Add user message
    const userMessage = {
      id: Date.now(),
      type: 'user',
      content: currentMessage,
      timestamp: new Date()
    }
    
    messages = [...messages, userMessage]
    const userInput = currentMessage
    currentMessage = ''
    isTyping = true
    
    // Simulate AI response
    setTimeout(() => {
      const aiResponse = generateAIResponse(userInput, selectedContext)
      const assistantMessage = {
        id: Date.now() + 1,
        type: 'assistant',
        content: aiResponse,
        timestamp: new Date()
      }
      
      messages = [...messages, assistantMessage]
      isTyping = false
    }, 1500)
  }
  
  function generateAIResponse(input: string, context: string) {
    const lowerInput = input.toLowerCase()
    
    if (context === 'code' || lowerInput.includes('code') || lowerInput.includes('generate')) {
      return `Here's a Python code example for your request:

\`\`\`python
import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score

# Load and preprocess data
df = pd.read_csv('your_dataset.csv')
X = df.drop('target', axis=1)
y = df['target']

# Split the data
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42
)

# Train model
model = RandomForestClassifier(n_estimators=100, random_state=42)
model.fit(X_train, y_train)

# Evaluate
predictions = model.predict(X_test)
accuracy = accuracy_score(y_test, predictions)
print(f"Accuracy: {accuracy:.3f}")
\`\`\`

This code provides a solid foundation. Would you like me to explain any specific part or suggest improvements?`
    }
    
    if (context === 'debug' || lowerInput.includes('error') || lowerInput.includes('debug')) {
      return `I can help you debug this issue. Here are common solutions:

**Common ML Debugging Steps:**
1. **Check data shapes**: Ensure X and y have compatible dimensions
2. **Handle missing values**: Use \`df.fillna()\` or \`SimpleImputer\`
3. **Encode categorical variables**: Use \`LabelEncoder\` or \`OneHotEncoder\`
4. **Scale features**: Apply \`StandardScaler\` or \`MinMaxScaler\`
5. **Check for data leakage**: Ensure no future information in features

**Error-specific solutions:**
- \`ValueError: Input contains NaN\`: Handle missing values first
- \`ValueError: could not convert string to float\`: Encode categorical data
- \`MemoryError\`: Reduce dataset size or use batch processing

Share your specific error message and I'll provide targeted help!`
    }
    
    if (context === 'analysis' || lowerInput.includes('analysis') || lowerInput.includes('data')) {
      return `For effective data analysis, I recommend this approach:

**1. Exploratory Data Analysis (EDA)**
\`\`\`python
# Basic statistics
df.describe()
df.info()
df.isnull().sum()

# Visualizations
import matplotlib.pyplot as plt
import seaborn as sns

# Distribution plots
df.hist(figsize=(12, 8))
plt.show()

# Correlation heatmap
plt.figure(figsize=(10, 8))
sns.heatmap(df.corr(), annot=True, cmap='coolwarm')
plt.show()
\`\`\`

**2. Key Insights to Look For:**
- Missing value patterns
- Outliers and anomalies
- Feature correlations
- Target variable distribution
- Class imbalance

Would you like me to dive deeper into any specific analysis technique?`
    }
    
    if (context === 'optimization' || lowerInput.includes('optimize') || lowerInput.includes('improve')) {
      return `Here are optimization strategies for better model performance:

**Hyperparameter Tuning:**
\`\`\`python
from sklearn.model_selection import GridSearchCV

param_grid = {
    'n_estimators': [100, 200, 300],
    'max_depth': [10, 20, None],
    'min_samples_split': [2, 5, 10]
}

grid_search = GridSearchCV(
    RandomForestClassifier(),
    param_grid,
    cv=5,
    scoring='accuracy'
)
grid_search.fit(X_train, y_train)
best_model = grid_search.best_estimator_
\`\`\`

**Feature Engineering:**
- Create polynomial features
- Apply feature selection (SelectKBest, RFE)
- Engineer domain-specific features
- Use feature scaling appropriately

**Model Ensemble:**
- Combine multiple algorithms
- Use voting classifiers
- Apply stacking techniques

Which optimization area interests you most?`
    }
    
    return `I understand you're asking about "${input}". Here are some suggestions:

**General ML Best Practices:**
- Start with simple models before complex ones
- Always validate your data quality first
- Use cross-validation for reliable metrics
- Document your experiments thoroughly
- Monitor for overfitting vs underfitting

**Next Steps:**
1. Define your problem clearly (classification/regression)
2. Explore and clean your data
3. Choose appropriate evaluation metrics
4. Iterate on feature engineering
5. Compare multiple algorithms

Is there a specific aspect you'd like me to elaborate on? I can provide code examples, explain concepts, or help debug issues.`
  }
  
  function useQuickPrompt(prompt: string) {
    currentMessage = prompt
  }
  
  function copyToClipboard(content: string) {
    navigator.clipboard.writeText(content)
  }
  
  function rateMessage(messageId: number, rating: 'up' | 'down') {
    console.log(`Rated message ${messageId} as ${rating}`)
  }
</script>

<div class="flex h-full">
  <!-- Chat Interface -->
  <div class="flex-1 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-dark-700">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold flex items-center">
          <Bot size={24} class="text-blue-400 mr-2" />
          AI Copilot
        </h2>
        
        <div class="flex items-center space-x-3">
          <select bind:value={selectedContext} class="input-field">
            {#each contextOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
          <div class="flex items-center text-sm text-dark-400">
            <Sparkles size={16} class="mr-1" />
            GPT-4 Powered
          </div>
        </div>
      </div>
    </div>
    
    <!-- Messages -->
    <div class="flex-1 overflow-auto p-4 space-y-4">
      {#each messages as message}
        <div class="flex {message.type === 'user' ? 'justify-end' : 'justify-start'}">
          <div class="max-w-3xl {message.type === 'user' ? 'bg-primary-600' : 'bg-dark-700'} rounded-lg p-4">
            <div class="flex items-start space-x-3">
              {#if message.type === 'assistant'}
                <Bot size={20} class="text-blue-400 mt-1 flex-shrink-0" />
              {/if}
              
              <div class="flex-1">
                <div class="prose prose-invert max-w-none">
                  {@html message.content.replace(/```(\w+)?\n([\s\S]*?)```/g, '<pre class="bg-dark-900 p-3 rounded text-sm overflow-x-auto"><code>$2</code></pre>').replace(/`([^`]+)`/g, '<code class="bg-dark-800 px-1 rounded text-sm">$1</code>').replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/\n/g, '<br>')}
                </div>
                
                {#if message.type === 'assistant'}
                  <div class="flex items-center justify-between mt-3 pt-3 border-t border-dark-600">
                    <div class="flex space-x-2">
                      <button 
                        class="p-1 hover:bg-dark-600 rounded"
                        on:click={() => copyToClipboard(message.content)}
                      >
                        <Copy size={14} class="text-dark-400" />
                      </button>
                      <button 
                        class="p-1 hover:bg-dark-600 rounded"
                        on:click={() => rateMessage(message.id, 'up')}
                      >
                        <ThumbsUp size={14} class="text-dark-400" />
                      </button>
                      <button 
                        class="p-1 hover:bg-dark-600 rounded"
                        on:click={() => rateMessage(message.id, 'down')}
                      >
                        <ThumbsDown size={14} class="text-dark-400" />
                      </button>
                    </div>
                    <span class="text-xs text-dark-500">
                      {message.timestamp.toLocaleTimeString()}
                    </span>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        </div>
      {/each}
      
      {#if isTyping}
        <div class="flex justify-start">
          <div class="bg-dark-700 rounded-lg p-4">
            <div class="flex items-center space-x-3">
              <Bot size={20} class="text-blue-400" />
              <div class="flex space-x-1">
                <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce"></div>
                <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
    
    <!-- Input -->
    <div class="p-4 border-t border-dark-700">
      <div class="flex space-x-3">
        <input
          type="text"
          bind:value={currentMessage}
          on:keydown={(e) => e.key === 'Enter' && sendMessage()}
          placeholder="Ask me anything about ML, coding, or data analysis..."
          class="input-field flex-1"
          disabled={isTyping}
        />
        <button 
          class="btn-primary"
          on:click={sendMessage}
          disabled={!currentMessage.trim() || isTyping}
        >
          <Send size={16} />
        </button>
      </div>
    </div>
  </div>
  
  <!-- Quick Actions Sidebar -->
  <div class="w-80 panel border-l border-dark-700 p-4">
    <h3 class="font-medium mb-4">Quick Prompts</h3>
    
    <div class="space-y-2 mb-6">
      {#each quickPrompts as prompt}
        <button
          class="w-full p-3 text-left text-sm bg-dark-700 hover:bg-dark-600 rounded-lg transition-colors"
          on:click={() => useQuickPrompt(prompt)}
        >
          {prompt}
        </button>
      {/each}
    </div>
    
    <h3 class="font-medium mb-4">Context Modes</h3>
    
    <div class="space-y-2">
      {#each contextOptions as option}
        <button
          class="w-full p-3 text-left rounded-lg transition-colors {selectedContext === option.value ? 'bg-blue-600' : 'bg-dark-700 hover:bg-dark-600'}"
          on:click={() => selectedContext = option.value}
        >
          <div class="flex items-center">
            <svelte:component this={option.icon} size={16} class="mr-2" />
            <span class="text-sm">{option.label}</span>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>