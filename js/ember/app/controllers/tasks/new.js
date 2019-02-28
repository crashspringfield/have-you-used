import Controller from '@ember/controller'

export default Controller.extend({
  actions: {
    addTask() {
      const title = this.get('title')
      const date = this.get('date')
      const description = this.get('description')

      const newTask = this.store.createRecord('task', {
        title,
        description,
        date: new Date(date)
      })

      newTask.save()

      this.setProperties({
        title: '',
        description: '',
        date: ''
      })
    }
  }
})
