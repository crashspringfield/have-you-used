import Controller from '@ember/controller'

export default Controller.extend({
  actions: {
    editTask(id) {
      const self = this
      const title = this.get('model.title')
      const date = this.get('model.date')
      const description = this.get('model.description')

      this.store.findRecord('task', id)
        .then(task => {
          task.set('title', title)
          task.set('date', new Date(date))
          task.set('description', description)

          task.save()
        })

      self.transitionToRoute('tasks')
    }
  }
})
